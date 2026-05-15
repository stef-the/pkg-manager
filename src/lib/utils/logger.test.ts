import { describe, it, expect, vi, beforeEach } from 'vitest';
import { createLogger } from './logger';

describe('logger', () => {
	beforeEach(() => {
		vi.restoreAllMocks();
		vi.spyOn(console, 'debug').mockImplementation(() => {});
		vi.spyOn(console, 'info').mockImplementation(() => {});
		vi.spyOn(console, 'warn').mockImplementation(() => {});
		vi.spyOn(console, 'error').mockImplementation(() => {});
	});

	it('creates a tagged logger', () => {
		const log = createLogger('test');
		expect(log).toHaveProperty('debug');
		expect(log).toHaveProperty('info');
		expect(log).toHaveProperty('warn');
		expect(log).toHaveProperty('error');
	});

	it('logs with tag prefix', () => {
		const log = createLogger('myModule');
		log.info('Hello');
		expect(console.info).toHaveBeenCalledWith('[myModule] Hello');
	});

	it('passes additional arguments', () => {
		const log = createLogger('test');
		log.error('Failed', { code: 42 });
		expect(console.error).toHaveBeenCalledWith('[test] Failed', { code: 42 });
	});

	it('each log level calls the correct console method', () => {
		const log = createLogger('lvl');
		log.debug('d');
		log.info('i');
		log.warn('w');
		log.error('e');
		expect(console.debug).toHaveBeenCalledTimes(1);
		expect(console.info).toHaveBeenCalledTimes(1);
		expect(console.warn).toHaveBeenCalledTimes(1);
		expect(console.error).toHaveBeenCalledTimes(1);
	});
});
