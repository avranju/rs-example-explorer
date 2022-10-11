import { invoke } from '@tauri-apps/api/tauri';

export interface Target {
    name: string;
    srcPath: string;
    kind: string[];
}

export interface Package {
    name: string;
    targets: Target[];
}

export async function openManifest(manifestPath: string) {
    await invoke('open_manifest', {
        manifestPath,
    });
}

async function listPackages(): Promise<Package[]> {
    return await invoke('list_packages');
}
