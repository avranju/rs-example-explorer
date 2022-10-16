<script lang="ts">
    import { slide } from 'svelte/transition';
    import { quintOut } from 'svelte/easing';
    import { onMount } from 'svelte';

    const mkey = (k: string) => `__node__${k}`;

    export const store = {
        set: (key: string, val: string) => {
            window.sessionStorage.setItem(mkey(key), val);
        },

        get: (key: string): string => {
            return window.sessionStorage.getItem(mkey(key));
        },

        get_or_default: (key: string, def: string): string =>
            store.get(key) || def,
    };

    const NodeState = Object.freeze({
        Expanded: '0',
        Collapsed: '1',
    });

    let state = NodeState.Collapsed;
    export let nestLevel = 0;
    export let id: string;

    // Save and retrieve node expand state during component destroy/mount.
    onMount(() => {
        state = store.get_or_default(id, NodeState.Collapsed);
        return () => {
            store.set(id, state);
        };
    });

    function toggleNodeState() {
        if (haveContent()) {
            state =
                state === NodeState.Collapsed
                    ? NodeState.Expanded
                    : NodeState.Collapsed;
        }
    }

    function haveContent() {
        return !!$$slots.content;
    }

    function onKeyPress() {
        // TODO: Toggle node state when return key is pressed. Also see how to make
        // node keyboard focusable.
    }
</script>

<div
    on:click={toggleNodeState}
    on:keypress={onKeyPress}
    class="grid grid-cols-10"
    style="cursor: pointer; margin-left: {nestLevel * 16}px"
>
    <div class={haveContent() ? 'col-span-9' : 'col-span-10'}>
        <slot name="header">
            <h1>No Header</h1>
        </slot>
    </div>

    {#if haveContent()}
        {#if state === NodeState.Collapsed}
            <div class="w-4 h-full flex items-center justify-center">
                <img src="right.svg" alt="right chevron" />
            </div>
        {:else}
            <div class="w-4 h-full flex items-center justify-center">
                <img src="down.svg" alt="left chevron" />
            </div>
        {/if}
    {/if}
</div>

{#if haveContent() && state === NodeState.Expanded}
    <div transition:slide={{ duration: 100, easing: quintOut }}>
        <slot name="content" />
    </div>
{/if}
