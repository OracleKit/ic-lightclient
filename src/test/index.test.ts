import {describe, expect, test} from '@jest/globals';
import { spawn, terminate } from './process';

describe('sum module', () => {
    beforeAll(async () => {
        await spawn('dfx', ['start']);
    })
    test('adds 1 + 2 to equal 3', async () => {
        await new Promise(resolve => setTimeout(resolve, 10*1000));
        expect(1+2).toBe(3);
    }, 15000);
    afterAll(async () => {
        await terminate();
    })
});