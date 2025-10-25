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
        let base_gas_fee = await actor.get_base_gas_fee();
        let max_priority_fee = await actor.get_max_priority_fee();

        await setupOcAgent();
        await new Promise(resolve => setTimeout(resolve, 30*1000));

        expect(await checkOcAgentHealthy()).toBe(true);
        let new_block_hash = await actor.get_latest_block_hash();
        let new_base_gas_fee = await actor.get_base_gas_fee();
        let new_max_priority_fee = await actor.get_max_priority_fee();

        expect(typeof block_hash).toBe('string');
        expect(block_hash !== new_block_hash).toBe(true);
        expect(base_gas_fee !== new_base_gas_fee).toBe(true);
        expect(max_priority_fee !== new_max_priority_fee).toBe(true);
        
    }, 120*1000);
    
    afterAll(async () => {
        await terminate();

    }, 20*1000)
});