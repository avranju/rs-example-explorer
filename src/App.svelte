<script lang="ts">
    import Welcome from './lib/Welcome.svelte';

    enum Pane {
        Welcome,
        Main,
    }

    let currentPane: Pane = Pane.Welcome;
    let manifestPath: string = '';

    function onOpenCargoFile({ detail }) {
        manifestPath = detail;
        currentPane = Pane.Main;
    }
</script>

<main class="container mx-auto">
    <div style="display: {currentPane === Pane.Welcome ? 'block' : 'none'}">
        <Welcome on:fileOpen={onOpenCargoFile} />
    </div>

    <div
        style="display: {currentPane === Pane.Main ? 'block' : 'none'}"
        class="grid grid-cols-2 gap-4"
    >
        <div>Cell 1</div>
        <div>Cell 2</div>
        <div>Cell 3</div>
        <div>Cell 4</div>
    </div>
</main>
