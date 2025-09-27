import { isChildHealthy, spawn } from "../process";

let processId: number | undefined;
const DFX_URL: string = 'http://localhost:4943';

export async function setupDfx(): Promise<string> {
    if ( processId != undefined ) return DFX_URL;

    processId  = await spawn('dfx', ['start', '--clean']);
    
    await new Promise<void>((resolve, reject) => {
        let retries = 0;

        const interval = setInterval(async () => {
            const isHealthy = await checkDfxHealthy();
            retries++;

            if ( retries < 10 && !isHealthy ) return;

            clearInterval(interval);
            isHealthy ? resolve() : reject();
            
        }, 1000)
    });

    return DFX_URL;
}

export async function checkDfxHealthy(): Promise<boolean> {
    if ( processId === undefined ) return false;
    if ( !isChildHealthy(processId!) ) return false;

    try {
        await fetch(DFX_URL);
        return true;
    } catch {
        return false;
    }
}