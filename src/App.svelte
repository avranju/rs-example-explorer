<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';

    let openStatus = 'Closed';
    let packageCount = 0;
    let packagesList = [];

    async function onOpenManifest() {
        try {
            openStatus = 'Opening';
            await invoke('open_manifest', {
                manifestPath: '/Users/avranju/code/bevy/Cargo.toml',
            });
            openStatus = 'Opened';
        } catch (err) {
            openStatus = err;
        }
    }

    async function onListPackages() {
        try {
            let packages: any = await invoke('list_packages');
            packageCount = packages.length;
            packagesList = packages;
        } catch (err) {
            openStatus = err;
        }
    }
</script>

<main>
    <h1>Rust Examples Explorer</h1>

    <button on:click={onOpenManifest}>Open Manifest</button>
    <h3>Open Status:</h3>
    <p>{openStatus}</p>

    <button on:click={onListPackages}>List Packages</button>
    <h3>List Packages:</h3>
    <p>{packageCount}</p>
    <ol>
        {#each packagesList as pkg}
            <li>{pkg.name}</li>
        {/each}
    </ol>
</main>
