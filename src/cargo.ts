import { invoke } from '@tauri-apps/api/tauri';

export interface Target {
    name: string;
    src_path: string;
    kind: string[];
}

export interface Package {
    id: string;
    name: string;
    version: string;
    targets: Target[];
}

export async function openManifest(manifestPath: string) {
    await invoke('open_manifest', {
        manifestPath,
    });
}

export async function listPackages(): Promise<Package[]> {
    return await invoke('list_packages');
}
