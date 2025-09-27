import {describe, expect, test} from '@jest/globals';
import { spawnAndWait, terminate } from './process';
import { setupDfx } from './components/dfx';
import { setupCanister } from './components/canister';
import { checkOcAgentHealthy, setupOcAgent } from './components/oc-agent';
import { createActor } from './declarations/canister';
import { HttpAgent } from '@dfinity/agent';

let agent: HttpAgent;

describe('test e2e', () => {
    beforeAll(async () => {
        let host = await setupDfx();
        agent = new HttpAgent({ host });
        await agent.fetchRootKey();

    }, 20*1000)
    
    test('block hash getting updated', async () => {
        let canisterId = await setupCanister();
        let actor = createActor(canisterId, { agent });
        let block_hash = await actor.get_latest_block_hash();

        await setupOcAgent();
        await new Promise(resolve => setTimeout(resolve, 30*1000));

        expect(await checkOcAgentHealthy()).toBe(true);
        let new_block_hash = await actor.get_latest_block_hash();

        expect(typeof block_hash).toBe('string');
        expect(block_hash !== new_block_hash).toBe(true);
        
    }, 60*1000);
    
    afterAll(async () => {
        await terminate();

    }, 20*1000)
});