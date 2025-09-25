import { ChildProcess, spawn as childSpawn } from "child_process";

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

        process.on('uncaughtException', async (err, origin) => {
            console.error("Uncaught exception:", err, origin);
            await this.terminate();
            process.exit(1);
        });

        process.on('unhandledRejection', async (reason, promise) => {
            console.error("Uncaught rejection:", reason, promise);
            await this.terminate();
            process.exit(1);
        });

        process.on('beforeExit', async () => {
            await this.terminate();
        });

        process.on('SIGINT', () => this.terminate());
        process.on('SIGTERM', () => this.terminate());
    }

    registerProcess(command: string, args: string[]): number {
        const child = childSpawn(command, args);
        const wrappedChild = new ProcessWrapper(child);

        child.stdout.on('data', this._pipeOutput(command));
        child.stderr.on('data', this._pipeOutput(command));
        
        child.on('spawn', () => {
            console.log(`${this._logPrefix} Running: ${command} ${args.join(' ')}`);
            wrappedChild.state = ProcessState.SPAWNED;
        });

        child.on('error', (e: Error) => {
            console.log(`${this._logPrefix} ${command} errored out: ${e}`);
            wrappedChild.isError = true;
        });

        child.on('exit', (code: number, signal) => {
            console.log(`${this._logPrefix} ${command} exited with code: ${code} ${signal}`);
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

        console.log(`${this._logPrefix} Running process termination.`);

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
                    console.log(`${this._logPrefix} Processes terminated.`);
                    resolve();
                }

                if ( ++retries == 25 ) {
                    targetProcs.forEach(p => p.process.kill('SIGKILL'));
                }
            }, 200);
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

                console.log(`[${command}] ${line}`);
            }

            buffer += chunkLines.shift() ?? '';
        }
    }
}

const STATE_UPDATE_CHECK_INTERVAL_DELAY: number = 200; // in milliseconds

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

        }, 200);
    });
}

export async function terminate() {
    ProcessManager.init();
    await ProcessManager.terminate();
}

export const ProcessManager = new ProcessManagerClass();