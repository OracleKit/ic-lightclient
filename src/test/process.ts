import { ChildProcess, spawn as childSpawn } from "child_process";

const STATE_UPDATE_CHECK_INTERVAL_DELAY: number = 200; // in milliseconds
const MAX_RETRIES = 25; // 25 * 200 = 5 secs

enum ProcessState {
    INIT,
    SPAWNED,
    EXITED
}

class ProcessWrapper {
    process: ChildProcess;
    state: ProcessState;
    isError: boolean;
    
    constructor(process_: ChildProcess) {
        this.process = process_;
        this.state = ProcessState.INIT;
        this.isError = false;
    }
}

// helps bypass jest log interception and 
// have pretty logs of the process manager.
class CustomConsole {
    static log(...args: any[]) {
        process.stdout.write(args.join(" ") + "\n");
    }

    static error(...args: any[]) {
        process.stderr.write(args.join(" ") + "\n");
    }
}

class ProcessManagerClass {
    private _children: ProcessWrapper[] = [];
    private readonly _logPrefix: string = "[ProcessManager]";
    private _terminationOngoing: boolean = false;
    private _init: boolean = false;

    init() {
        if ( this._init ) return;
        this._init = true;

        this._children = [];
        this._terminationOngoing = false;

        process.on('uncaughtException', (err, origin) => {
            CustomConsole.error("Uncaught exception:", err, origin);
            return this.terminate(true);
        });

        process.on('unhandledRejection', (reason, promise) => {
            CustomConsole.error("Uncaught rejection:", reason, promise);
            return this.terminate(true);
        });

        process.on('beforeExit', () => this.terminate());
        process.on('SIGINT', () => this.terminate(true));
        process.on('SIGTERM', () => this.terminate(true));
    }

    registerProcess(command: string, args: string[]): number {
        const child = childSpawn(command, args);
        const wrappedChild = new ProcessWrapper(child);

        child.stdout.on('data', this._pipeOutput(command));
        child.stderr.on('data', this._pipeOutput(command));
        
        child.on('spawn', () => {
            CustomConsole.log(`${this._logPrefix} Running: ${command} ${args.join(' ')}`);
            wrappedChild.state = ProcessState.SPAWNED;
        });

        child.on('error', (e: Error) => {
            CustomConsole.log(`${this._logPrefix} ${command} errored out: ${e}`);
            wrappedChild.isError = true;
        });

        child.on('exit', (code: number, signal) => {
            CustomConsole.log(`${this._logPrefix} ${command} exited with code: ${code} ${signal}`);
            wrappedChild.state = ProcessState.EXITED;
        });

        return this._children.push(wrappedChild) - 1;
    }

    getChildProcess(id: number): ProcessWrapper {
        return this._children[id];
    }

    async terminate(exit: boolean = false) {
        if ( this._terminationOngoing ) return;
        this._terminationOngoing = true;

        CustomConsole.log(`${this._logPrefix} Running process termination.`);

        let targetProcs = this._children.filter(p => p.state != ProcessState.EXITED);
        targetProcs.forEach(p => {
            p.isError = false;
            p.process.kill();
        });

        await new Promise<void>(resolve => {
            let retries = 0;

            const interval = setInterval(() => {
                targetProcs = targetProcs.filter(p => (
                    p.state != ProcessState.EXITED && !p.isError
                ));

                if ( targetProcs.length == 0 ) {
                    clearInterval(interval);
                    CustomConsole.log(`${this._logPrefix} Processes terminated.`);
                    resolve();
                }

                if ( ++retries == MAX_RETRIES ) {
                    targetProcs.forEach(p => p.process.kill('SIGKILL'));
                }

                if ( retries == 2*MAX_RETRIES ) {
                    clearInterval(interval);
                    CustomConsole.log(`${this._logPrefix} ${targetProcs.length} could not be terminated.`);
                    resolve();
                }
            }, STATE_UPDATE_CHECK_INTERVAL_DELAY);
        });

        if ( exit ) {
            process.exit(1);
        }
    }

    private _pipeOutput(command: string) {
        let buffer = "";

        return (chunk: any) => {
            const chunkStr = `${chunk}`;
            const chunkLines = chunkStr.split('\n');

            while ( chunkLines.length > 1 ) {
                const line = buffer + (chunkLines.shift() ?? '');
                buffer = "";

                CustomConsole.log(`[${command}] ${line}`);
            }

            buffer += chunkLines.shift() ?? '';
        }
    }
}

export function init() {
    ProcessManager.init();
}

export async function spawn(command: string, args: string[]): Promise<number> {
    ProcessManager.init();
    const childId = ProcessManager.registerProcess(command, args);
    const child = ProcessManager.getChildProcess(childId);

    return new Promise((resolve, reject) => {
        const interval = setInterval(() => {
            if ( child.state == ProcessState.INIT && !child.isError ) return;

            clearInterval(interval);

            if ( child.isError ) return reject();
            if ( child.state == ProcessState.EXITED ) return reject();
            
            // child.state == ProcessState.SPAWNED
            resolve(childId);

        }, STATE_UPDATE_CHECK_INTERVAL_DELAY);
    });
}

export function isChildHealthy(childId: number): boolean {
    ProcessManager.init();
    const child = ProcessManager.getChildProcess(childId);
    return child.state == ProcessState.SPAWNED && !child.isError;
}

export async function spawnSync(command: string, args: string[]): Promise<number | null> {
    ProcessManager.init();
    const childId = ProcessManager.registerProcess(command, args);
    const child = ProcessManager.getChildProcess(childId);

    return new Promise((resolve, reject) => {
        const interval = setInterval(() => {
            if ( child.state != ProcessState.EXITED && !child.isError ) return;

            clearInterval(interval);

            if ( child.isError ) return reject();
            resolve(child.process.exitCode);

        }, STATE_UPDATE_CHECK_INTERVAL_DELAY);
    });
}

export async function terminate() {
    ProcessManager.init();
    await ProcessManager.terminate();
}

export const ProcessManager = new ProcessManagerClass();