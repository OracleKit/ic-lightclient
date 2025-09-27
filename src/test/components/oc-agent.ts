import { isChildHealthy, spawn } from "../process";

let processId: number | undefined;

export async function setupOcAgent() {
    processId = await spawn('bash', ['-c', './target/debug/ic-lightclient-oc-agent']);
}

export async function checkOcAgentHealthy(): Promise<boolean> {
    if ( processId === undefined ) return false;
    if ( !isChildHealthy(processId!) ) return false;
    return true;
}