<script lang="ts">
    import Welcome from './lib/Welcome.svelte';

    enum Pane {
        Welcome,
        Main,
    }

    let currentPane: Pane = Pane.Main;
    let manifestPath: string = '';

    function onOpenCargoFile({ detail }) {
        manifestPath = detail;
        currentPane = Pane.Main;
    }
</script>

<main>
    <div style="display: {currentPane === Pane.Welcome ? 'block' : 'none'}">
        <Welcome on:fileOpen={onOpenCargoFile} />
    </div>

    <div
        style="display: {currentPane === Pane.Main ? 'block' : 'none'}"
        class="grid grid-cols-2 gap-1 bg-gradient-to-br from-gray-700 to-gray-800 w-screen h-screen"
    >
        <div>1</div>
        <div>a2</div>
    </div>

    <!-- <div
        style="display: {currentPane === Pane.Main ? 'block' : 'none'}"
        class="grid grid-cols-4 gap-1 bg-gradient-to-br from-gray-700 to-gray-800 h-screen w-screen"
    >
        <div class="bg-slate-700">
            <div>
                <h1
                    class="border-b-4 border-gray-400 bg-gradient-to-r from-slate-700 to-slate-800 py-2 pr-2 text-right text-xl text-white"
                >
                    Packages
                </h1>
                <div class="text-slate-300">
                    <div
                        class="grid cursor-pointer grid-cols-8 border-b-2 border-gray-500 pb-3 hover:bg-slate-800"
                    >
                        <div class="row-span-2 place-self-center">1</div>
                        <div class="col-span-7 text-lg font-bold">quote</div>
                        <div class="col-span-7 ml-4 text-sm">v1.0.21</div>
                    </div>
                    <div
                        class="grid cursor-pointer grid-cols-8 border-b-2 border-gray-500 pb-3 hover:bg-slate-800"
                    >
                        <div class="row-span-2 place-self-center">2</div>
                        <div class="col-span-7 text-lg font-bold">
                            proc-macro2
                        </div>
                        <div class="col-span-7 ml-4 text-sm">v1.0.46</div>
                    </div>
                    <div
                        class="grid cursor-pointer grid-cols-8 border-b-2 border-gray-500 pb-3 hover:bg-slate-800"
                    >
                        <div class="row-span-2 place-self-center">3</div>
                        <div class="col-span-7 text-lg font-bold">bevy</div>
                        <div class="col-span-7 ml-4 text-sm">v0.8.1</div>
                    </div>
                </div>
            </div>
        </div>
        <div class="col-span-3">
            <h1
                class="border-b-4 border-gray-400 bg-gradient-to-l from-slate-700 to-slate-800 py-2 pr-2 text-right text-xl text-gray-400"
            >
                Examples
            </h1>
            <div class="text-slate-100">
                <div class="grid grid-cols-4 gap-6 p-4">
                    <div
                        class="h-28 cursor-pointer rounded-md bg-slate-500 text-center shadow-md shadow-black hover:bg-slate-600"
                    >
                        <h2
                            class="truncate text-xl text-left bg-gray-900 pl-2 py-1"
                        >
                            hello_world
                        </h2>
                        <div
                            class="truncate text-sm text-left pl-2 pb-2 pt-1 text-gray-300"
                        >
                            examples/hello_world.rs
                        </div>
                        <div class="text-right">
                            <button class="bg-slate-800 rounded-tl-md"
                                >Run</button
                            >
                        </div>
                    </div>
                    <div
                        class="h-28 cursor-pointer rounded-md bg-slate-500 text-center shadow-md shadow-black hover:bg-slate-600"
                    >
                        02
                    </div>
                    <div
                        class="h-28 cursor-pointer rounded-md bg-slate-500 text-center shadow-md shadow-black hover:bg-slate-600"
                    >
                        03
                    </div>
                    <div
                        class="h-28 cursor-pointer rounded-md bg-slate-500 text-center shadow-md shadow-black hover:bg-slate-600"
                    >
                        04
                    </div>
                    <div
                        class="h-28 cursor-pointer rounded-md bg-slate-500 text-center shadow-md shadow-black hover:bg-slate-600"
                    >
                        05
                    </div>
                    <div
                        class="h-28 cursor-pointer rounded-md bg-slate-500 text-center shadow-md shadow-black hover:bg-slate-600"
                    >
                        06
                    </div>
                </div>
            </div>
        </div>
    </div> -->
</main>
