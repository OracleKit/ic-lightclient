import { tmpdir } from "os";
import { spawnAndWait } from "../process";
import { realpathSync } from "fs";
import { readFile } from "fs/promises";
import { join } from "path";

let canisterId: string;

export async function setupCanister(): Promise<string> {
    let ret = await spawnAndWait('dfx', ['deploy', 'canister']);
    if ( ret !== 0 ) throw "Unable to deploy canister.";

    const tempDirPath = tmpdir();
    const resolvedTempDirPath = realpathSync(tempDirPath);
    const canisterFile = join(resolvedTempDirPath, 'ORACLEKIT_TMP_canister');

    ret = await spawnAndWait('bash', ['-c', `dfx canister id canister > ${canisterFile}`]);
    if ( ret !== 0 ) throw "Unable to get canister id.";

    canisterId = (await readFile(canisterFile, { encoding: 'utf-8' })).trim();

    ret = await spawnAndWait('rm', [canisterFile]);
    if ( ret !== 0 ) throw "Unable to cleanup.";

    ret = await spawnAndWait('dfx', ['canister', 'call', "canister", "init"]);
    if ( ret !== 0 ) throw "Unable to init canister";

    return canisterId;
}
