import {describe, expect, test} from '@jest/globals';
import { spawnAndWait, terminate } from './process';
import { setupDfx } from './components/dfx';
import { setupCanister } from './components/canister';
import { checkOcAgentHealthy, setupOcAgent } from './components/oc-agent';
import { createActor } from './declarations/canister';
import { HttpAgent } from '@dfinity/agent';

let agent: HttpAgent;

const EthereumMainnetChainId = 1;
const EthereumHoleskyChainId = 17000;

describe('test e2e', () => {
    beforeAll(async () => {
        let host = await setupDfx();
        agent = new HttpAgent({ host });
        await agent.fetchRootKey();

    }, 20*1000)
    
    test('block hash getting updated', async () => {
        let canisterId = await setupCanister();
        let actor = createActor(canisterId, { agent });
        let mainnet = {
            block_hash: await actor.get_latest_block_hash(EthereumMainnetChainId),
            base_gas_fee: await actor.get_base_gas_fee(EthereumMainnetChainId),
            max_priority_fee: await actor.get_max_priority_fee(EthereumMainnetChainId)
        };

        let holesky = {
            block_hash: await actor.get_latest_block_hash(EthereumHoleskyChainId),
            base_gas_fee: await actor.get_base_gas_fee(EthereumHoleskyChainId),
            max_priority_fee: await actor.get_max_priority_fee(EthereumHoleskyChainId)
        };

        await setupOcAgent();
        await new Promise(resolve => setTimeout(resolve, 30*1000));

        expect(await checkOcAgentHealthy()).toBe(true);
        
        let mainnet_new = {
            block_hash: await actor.get_latest_block_hash(EthereumMainnetChainId),
            base_gas_fee: await actor.get_base_gas_fee(EthereumMainnetChainId),
            max_priority_fee: await actor.get_max_priority_fee(EthereumMainnetChainId)
        };

        let holesky_new = {
            block_hash: await actor.get_latest_block_hash(EthereumHoleskyChainId),
            base_gas_fee: await actor.get_base_gas_fee(EthereumHoleskyChainId),
            max_priority_fee: await actor.get_max_priority_fee(EthereumHoleskyChainId)
        };

        expect(typeof mainnet.block_hash).toBe('string');
        expect(mainnet.block_hash !== mainnet_new.block_hash).toBe(true);
        expect(mainnet.base_gas_fee !== mainnet_new.base_gas_fee).toBe(true);
        expect(mainnet.max_priority_fee !== mainnet_new.max_priority_fee).toBe(true);

        expect(holesky.block_hash !== holesky_new.block_hash).toBe(true);
        expect(holesky.base_gas_fee !== holesky_new.base_gas_fee).toBe(true);
        expect(holesky.max_priority_fee !== holesky_new.max_priority_fee).toBe(true);
        
    }, 120*1000);
    
    afterAll(async () => {
        await terminate();

    }, 20*1000)
});