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
    manifest_path: string;
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

export enum FolderType {
    Package,
    Folder,
}

export class Folder {
    public id: string;
    public type: FolderType;
    public name: string;
    public version?: string;
    public targets: Target[];
    public folders: Folder[];
}

export function countTargets(folder: Folder): number {
    return (
        folder.targets.length +
        folder.folders.reduce((pv, f) => pv + countTargets(f), 0)
    );
}

export function locateFolder(root: Folder, id: string): Folder {
    if (root.id === id) {
        return root;
    }

    for (const f of root.folders) {
        const found = locateFolder(f, id);
        if (!!found) {
            return found;
        }
    }

    return null;
}

function addRootFolders(folder: Folder): Folder {
    // recursively do this processing for all child folders
    for (const f of folder.folders) {
        addRootFolders(f);
    }

    // if this folder has child folders and it has targets then add a new
    // child folder called '/' and move the targets to this new child
    if (folder.folders.length > 0 && folder.targets.length > 0) {
        folder.folders.splice(0, 0, {
            id: generateId(),
            type: FolderType.Folder,
            name: '/',
            targets: folder.targets,
            folders: [],
        });
        folder.targets = [];
    }

    return folder;
}

export function groupByFolder(pkg: Package, targets: Target[]): Folder {
    let packageFolder: Folder = {
        id: generateId(),
        type: FolderType.Package,
        name: pkg.name,
        version: pkg.version,
        targets: [],
        folders: [],
    };

    for (const target of targets) {
        const components = target.src_path.split('/');
        if (components.length > 1) {
            // discard the file name from the path
            components.pop();

            // create folders as needed
            let f = packageFolder;
            for (const fname of components) {
                f = findOrMakeFolder(f, fname);
            }

            f.targets.push(target);
        } else {
            // the example is in the root folder
            packageFolder.targets.push(target);
        }
    }

    return addRootFolders(packageFolder);
}

function findOrMakeFolder(folder: Folder, name: string): Folder {
    let f = folder.folders.find((f) => f.name === name);
    if (!f) {
        f = {
            id: generateId(),
            type: FolderType.Folder,
            name,
            targets: [],
            folders: [],
        };
        folder.folders.push(f);
    }

    return f;
}

function stripCargoToml(path: string): string {
    const CARGO_TOML = /[\\/]Cargo\.toml$/g;
    const res = CARGO_TOML.exec(path);
    if (!!res) {
        return path.substring(0, res.index);
    }

    return path;
}

function getExampleRelativePath(basePath: string, examplePath: string): string {
    const EXAMPLES_RE = /examples[\\/]+/g;

    // strip 'examples/' from the begining of the path
    let path = examplePath.substring(stripCargoToml(basePath).length + 1);
    if (EXAMPLES_RE.test(path)) {
        return path.substring(EXAMPLES_RE.lastIndex);
    }

    return path;
}

export function listExamples(pkg: Package, exampleFilter: string): Target[] {
    return pkg.targets
        .filter((t) => {
            let filter_str = exampleFilter.toLowerCase();
            return (
                t.kind.indexOf('example') !== -1 &&
                (t.name.toLowerCase().indexOf(filter_str) !== -1 ||
                    t.src_path.toLowerCase().indexOf(filter_str) !== -1)
            );
        })
        .map((t) => {
            return {
                name: t.name,
                kind: t.kind,
                src_path: getExampleRelativePath(pkg.manifest_path, t.src_path),
            };
        });
}

const USED_IDS: string[] = [];
function generateId(): string {
    let id = '';
    while (
        USED_IDS.indexOf((id = Math.random().toString(26).substring(2))) !== -1
    );

    USED_IDS.push(id);

    return id;
}
