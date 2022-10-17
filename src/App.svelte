<script lang="ts">
    import Welcome from './lib/Welcome.svelte';
    import {
        openManifest,
        listPackages,
        type Package,
        type Target,
        groupByFolder,
        Folder,
        listExamples,
        locateFolder,
    } from './cargo';
    import Node from './lib/Node.svelte';

    enum Pane {
        Welcome,
        Loading,
        Main,
    }

    let currentPane: Pane = Pane.Welcome;
    let manifestPath: string = '';
    let folders: Folder[] = [];
    let currentFolder: Folder = null;
    let exampleFilter: string = '';
    let selectedFolderId: string = '';

    async function onOpenCargoFile({ detail }) {
        manifestPath = detail;
        currentPane = Pane.Loading;

        // we do the actual work on the next tick because otherwise
        // the "Loading..." pane doesn't get rendered (yeah, in spite
        // of the 'await' in the code it still hangs :shrug:)
        setTimeout(async () => {
            await openManifest(manifestPath);
            currentFolder = null;
            folders = (await listPackages())
                .map((pkg) => [pkg, listExamples(pkg, exampleFilter)])
                .filter(([_, examples]) => (examples as Target[]).length > 0)
                .map(([pkg, examples]) => {
                    return groupByFolder(pkg as Package, examples as Target[]);
                });
            currentPane = Pane.Main;
        }, 1);
    }

    function onCloseCargoFile() {
        currentPane = Pane.Welcome;
        manifestPath = '';
        folders = [];
        currentFolder = null;
    }

    async function onFilterChange() {}

    function runExample(target: Target) {
        console.log('Running ' + target.name);
    }

    function onSelectedFolderChange(event) {
        selectedFolderId = event.detail.id;

        currentFolder = null;
        for (const folder of folders) {
            currentFolder = locateFolder(folder, selectedFolderId);
            if (!!currentFolder) {
                break;
            }
        }
    }

    (async () => {
        await onOpenCargoFile({
            detail: 'C:\\Code\\nannou\\Cargo.toml',
        });
    })();
</script>

<main>
    {#if currentPane === Pane.Welcome}
        <div class="grid">
            <Welcome on:fileOpen={onOpenCargoFile} />
        </div>
    {:else if currentPane === Pane.Loading}
        <h1 class="grid text-xl text-white">Loading...</h1>
    {:else}
        <div
            class="grid grid-cols-4 gap-1 bg-gray-800 h-screen w-screen overflow-hidden"
        >
            <div class="flex flex-col bg-slate-700 overflow-hidden">
                <h1
                    class="flex-none border-b-4 border-gray-400 bg-gradient-to-r from-slate-700 to-slate-800 py-2 pr-2 text-right text-xl text-white"
                >
                    Packages
                </h1>
                {#if folders.length > 0}
                    <div
                        class="text-slate-300 h-screen overflow-y-scroll overscroll-contain"
                    >
                        {#each folders as folder}
                            <div
                                class="cursor-pointer border-b-2 border-gray-500 pb-4"
                            >
                                <Node
                                    {folder}
                                    {selectedFolderId}
                                    on:selected={onSelectedFolderChange}
                                />
                            </div>
                        {/each}
                    </div>
                {:else}
                    <div class="grow place-self-center pt-2 text-slate-400">
                        No packages with examples found.
                    </div>
                {/if}
            </div>
            <div class="col-span-3 overflow-hidden">
                <div class="grid grid-cols-8 border-b-4 border-gray-400">
                    <h1
                        class="col-span-4 bg-slate-800 text-xl text-gray-400 p-2 text-ellipsis overflow-hidden"
                        title={manifestPath}
                    >
                        {manifestPath}
                    </h1>
                    <input
                        type="text"
                        placeholder="Filter Examples"
                        class="col-span-3 pl-2 mr-4"
                        bind:value={exampleFilter}
                        on:input={onFilterChange}
                    />
                    <h1
                        class="bg-gradient-to-l from-slate-700 to-slate-800 text-right text-xl text-gray-400"
                    >
                        <div
                            class="w-8 h-8 bg-transparent float-right py-2 pr-2 cursor-pointer"
                            title="Close Cargo File"
                            on:click={onCloseCargoFile}
                        >
                            <img src="close.svg" alt="Close" />
                        </div>
                    </h1>
                </div>

                <div class="text-slate-100 grid grid-cols-4 gap-6 p-4 pb-28">
                    {#if currentFolder !== null}
                        {#each currentFolder.targets as eg}
                            <div
                                class="h-28 cursor-pointer rounded-md bg-slate-600 text-center shadow-md shadow-black hover:bg-slate-700"
                                title="Run Example"
                                on:click={() => runExample(eg)}
                            >
                                <h2
                                    class="truncate text-lg text-center bg-gray-900 pr-2 py-1"
                                    title={eg.name}
                                >
                                    {eg.name}
                                </h2>
                                <div
                                    class="truncate text-sm text-left pl-2 pb-2 pt-1 text-gray-300"
                                    title={eg.src_path}
                                >
                                    {eg.src_path}
                                </div>
                                <div class="text-right">
                                    <!-- <button class="bg-slate-800 rounded-tl-md"
                                    ><img src="play.svg" alt="Play" /></button
                                > -->
                                    <div class="float-right mt-2">
                                        <img
                                            src="play.svg"
                                            alt="Play"
                                            class="w-8 h-8"
                                        />
                                    </div>
                                </div>
                            </div>
                        {/each}
                    {:else}
                        <div>--</div>
                    {/if}
                </div>
            </div>
        </div>
    {/if}
</main>
