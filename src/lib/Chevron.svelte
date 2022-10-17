<script lang="ts" context="module">
    export enum Type {
        Right,
        Down,
    }
</script>

<script lang="ts">
    export let type;
    $: image_type = type === Type.Right ? 'right' : 'down';
    $: image_source = `${image_type}.svg`;
    $: image_alt = `${image_type} chevron`;
</script>

<div class="w-full h-full opacity-50">
    <img src={image_source} class="w-4 h-4 float-right" alt={image_alt} />
</div>
