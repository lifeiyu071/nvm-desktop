import { invoke } from '@tauri-apps/api/core';

/**
 * @description: Get app settings data
 * @return {Promise<Nvmd.Setting>} Promise-Nvmd.Setting
 */
export async function getSettings() {
	return invoke<Nvmd.Setting>('read_settings');
}

/**
 * @description: Update app settings data
 * @param {Nvmd.Setting} settings app settings data
 * @return {Promise<void>} Promise-void
 */
export async function updateSettings(settings: Nvmd.Setting) {
	return invoke<void>('update_settings', { settings });
}

/**
 * @description: Get the current global node version
 * @param {boolean} fetch whether to read the latest value from the file
 * @return {Promise<string>} node version
 */
export function vCurrent(fetch: boolean = false) {
	return invoke<string>('current', { fetch });
}

/**
 * @description: Set the globally effective node version
 * @param {string} version node version number
 * @return {Promise<void>}	Promise-void
 */
export function vSetCurrent(version: string) {
	return invoke<void>('set_current', { version });
}

/**
 * @description: Get a list of all officially released versions of node
 * @param {boolean} fetch Whether to pull new data from remote services
 * @return {Promise<Nvmd.Versions>}	List of all officially released versions of node
 */
export function versionList(fetch: boolean = false) {
	return invoke<Nvmd.Versions>('version_list', { fetch });
}

/**
 * @description: Get a list of installed nodes
 * @param {boolean} fetch Whether to read the latest data
 * @return {Promise<Array<string>>} An array containing the node version number
 */
export function installedList(fetch: boolean = false) {
	return invoke<Array<string>>('installed_list', { fetch });
}

/**
 * @description: Download Node
 * @param {string} version node version
 * @param {string} arch	node architecture
 * @return {Promise<string>}	The file path where the downloaded node is saved
 */
export function installNode(version: string, arch?: string) {
	return invoke<string>('install_node', { version, arch });
}

/**
 * @description: Cancel the task of downloading node
 * @return {Promise<void>} Promise-void
 */
export function installNodeCancel() {
	return invoke<void>('install_node_cancel');
}

/**
 * @description	uninstall node
 * @param {string} version version number
 * @param {boolean} current whether the version to be uninstalled is the currently used version
 * @returns {Promise<void>} Promise<void>
 */
export function uninstallNode(version: string, current: boolean = false) {
	return invoke<void>('uninstall_node', { version, current });
}

/**
 * @description: Get project list from locale file
 * @param {boolean} fetch whether to read the latest value from the file
 * @return {Promise<Nvmd.Project[]>}
 */
export function projectList(fetch: boolean = false) {
	return invoke<Nvmd.Project[]>('project_list', { fetch });
}

/**
 * @description: Get group list from locale file
 * @param {boolean} fetch whether to read the latest value from the file
 * @return {Promise<Nvmd.Group[]>}
 */
export function groupList(fetch: boolean = false) {
	return invoke<Nvmd.Group[]>('group_list', { fetch });
}
