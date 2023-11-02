import {mockIPC, clearMocks} from '@tauri-apps/api/mocks'
import { invoke } from '@tauri-apps/api/tauri'
import { mockWindows } from "@tauri-apps/api/mocks";
import { getCurrent } from "@tauri-apps/api/window";

import {afterEach, test, expect, beforeAll} from 'vitest'

afterEach(() => clearMocks())


beforeAll(() => {
    Object.defineProperty(window, 'window', {});
});


test('save_event', async () => {
    mockWindows("main");
    mockIPC((cmd, args) => {
        expect(cmd).toBe('save_event');
        expect(args).toEqual({event: 'test_event'});
        return Promise.resolve();
    })

    expect(invoke('save_event', {event: 'test_event'})).resolves.toBe(undefined);
});