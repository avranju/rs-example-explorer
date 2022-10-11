<script lang="ts">
    import { open } from '@tauri-apps/api/dialog';
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    async function onOpenCargoFile() {
        const selected = await open({
            filters: [
                {
                    name: 'Cargo manifest',
                    extensions: ['toml'],
                },
            ],
        });

        if (!!selected) {
            // raise file open event
            dispatch('fileOpen', selected);
        }
    }
</script>

<button class="text-3xl" on:click={onOpenCargoFile}
    >Open <code>Cargo.toml</code>...</button
>
