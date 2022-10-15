<script lang="ts">
    import Welcome from './lib/Welcome.svelte';
    import {
        openManifest,
        listPackages,
        type Package,
        type Target,
    } from './cargo';

    enum Pane {
        Welcome,
        Loading,
        Main,
    }

    let currentPane: Pane = Pane.Welcome;
    let manifestPath: string = '';
    let packages: Package[] = [];
    let examples: Target[] = [];
    let currentPackage: Package = null;
    let exampleFilter: string = '';

    async function onOpenCargoFile({ detail }) {
        manifestPath = detail;
        currentPane = Pane.Loading;

        // we do the actual work on the next tick because otherwise
        // the "Loading..." pane doesn't get rendered (yeah, in spite
        // of the 'await' in the code it still hangs :shrug:)
        setTimeout(async () => {
            await openManifest(manifestPath);
            packages = (await listPackages()).filter(
                (pkg) => listExamples(pkg).length > 0
            );
            currentPackage = packages.length > 0 ? packages[0] : null;
            currentPane = Pane.Main;

            if (!!currentPackage) {
                await onListExamples(currentPackage);
            }
        }, 1);
    }

    function onCloseCargoFile() {
        currentPane = Pane.Welcome;
        manifestPath = '';
        packages = [];
        examples = [];
        currentPackage = null;
    }

    async function onFilterChange() {
        if (!!currentPackage) {
            await onListExamples(currentPackage);
        }
    }

    function getManifestDir(path: string): string {
        // strip '/Cargo.toml' from the end of the path
        if (path.endsWith('/Cargo.toml')) {
            return path.substring(0, path.length - 11);
        }

        return path;
    }

    function getExampleRelativePath(
        pkg_path: string,
        example_path: string
    ): string {
        // strip 'examples/' from the begining of the path
        let path = example_path.substring(getManifestDir(pkg_path).length + 1);
        if (path.startsWith('examples/')) {
            return path.substring(9);
        }

        return path;
    }

    function listExamples(pkg: Package): Target[] {
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
                    src_path: getExampleRelativePath(
                        pkg.manifest_path,
                        t.src_path
                    ),
                };
            });
    }

    function examplesCount(pkg: Package): string {
        let eg = listExamples(pkg);
        return eg.length > 0
            ? eg.length === 1
                ? `${eg.length} example`
                : `${eg.length} examples`
            : '';
    }

    async function onListExamples(pkg: Package) {
        examples = listExamples(pkg);
        currentPackage = pkg;
    }

    function runExample(target: Target) {
        console.log('Running ' + target.name);
    }
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
                {#if packages.length > 0}
                    <div
                        class="text-slate-300 h-screen overflow-y-scroll overscroll-contain"
                    >
                        {#each packages as pkg}
                            <div
                                class="grid cursor-pointer grid-cols-1 border-b-2 border-gray-500 pl-2 pb-3 hover:bg-slate-800 {pkg.id ===
                                currentPackage.id
                                    ? 'bg-slate-800'
                                    : 'bg-slate-700'}"
                                on:click={() => onListExamples(pkg)}
                            >
                                <div class="text-lg font-bold">
                                    {pkg.name}
                                </div>
                                <div class="ml-4 text-sm">
                                    {pkg.version}
                                </div>
                                <div class="ml-4 text-sm text-right mr-2">
                                    {examplesCount(pkg)}
                                </div>
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
                        class="col-span-4 bg-slate-800 text-xl text-gray-400 p-2"
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

                <div
                    class="text-slate-100 grid grid-cols-4 gap-6 p-4 h-screen overflow-y-scroll overscroll-contain pb-28"
                >
                    {#if packages.length > 0}
                        {#each examples as eg}
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
