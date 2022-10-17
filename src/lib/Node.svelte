<script lang="ts">
    import { slide } from 'svelte/transition';
    import { quintOut } from 'svelte/easing';
    import { onMount, createEventDispatcher } from 'svelte';

    import Chevron, { Type as ChevronType } from './Chevron.svelte';
    import { countTargets, FolderType, type Folder } from '../cargo';

    const NodeState = Object.freeze({
        Expanded: '0',
        Collapsed: '1',
    });

    const SelectionState = Object.freeze({
        Selected: '0',
        NotSelected: '1',
    });

    const dispatch = createEventDispatcher();

    //////////////////////////////////////
    /// Component state.
    ///
    export let folder: Folder;
    let state = NodeState.Collapsed;

    // currently selected folder in the entire tree set by the parent
    export let selectedFolderId: string;
    $: selectionState =
        selectedFolderId === folder.id
            ? SelectionState.Selected
            : SelectionState.NotSelected;

    // this node is selectable only if there are no child folders
    $: selectable = folder.folders.length === 0;
    $: nodeBackground =
        selectable && selectionState === SelectionState.Selected
            ? 'backdrop-brightness-150'
            : '';
    //////////////////////////////////////

    // Save and retrieve node expand state during component destroy/mount.
    onMount(() => {
        state = store.get_or_default(folder.id, NodeState.Collapsed);
        selectionState = store.get_or_default(
            `${folder.id}-sel`,
            SelectionState.NotSelected
        );
        return () => {
            store.set(folder.id, state);
            store.set(`${folder.id}-sel`, selectionState);
        };
    });

    function forwardSelected(event) {
        dispatch('selected', event.detail);
    }

    function toggleNodeState() {
        if (selectable) {
            dispatch('selected', { id: folder.id });
        }

        if (haveChildFolders()) {
            state =
                state === NodeState.Collapsed
                    ? NodeState.Expanded
                    : NodeState.Collapsed;
        }
    }

    function haveChildFolders(): boolean {
        return folder.folders.length > 0;
    }

    function onKeyPress() {
        // TODO: Toggle node state when return key is pressed. Also see how to make
        // node keyboard focusable.
    }

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
</script>

<div
    on:click={toggleNodeState}
    on:keypress={onKeyPress}
    class="grid grid-cols-10 cursor-pointer {nodeBackground} hover:backdrop-brightness-50"
>
    <div class="col-span-9">
        {#if folder.type === FolderType.Package}
            <div>
                <div class="text-lg font-bold ml-2">
                    {folder.name}
                </div>
                <div class="text-sm ml-8">
                    {folder.version}
                </div>
                <div class="text-sm text-gray-400 ml-8">
                    {`${countTargets(folder)} examples`}
                </div>
            </div>
        {:else if haveChildFolders()}
            <div class="text-sm">
                {folder.name}
                <span class="text-sm text-gray-400 float-right mr-2"
                    >{`${countTargets(folder)} examples`}</span
                >
            </div>
        {:else}
            <div class="text-sm">
                {folder.name}
                {#if countTargets(folder) > 1}
                    <span class="text-sm text-gray-400 float-right mr-2"
                        >{`${countTargets(folder)} examples`}</span
                    >
                {:else}
                    <span class="text-sm text-gray-400 float-right mr-2"
                        >1 example</span
                    >
                {/if}
            </div>
        {/if}
    </div>

    <div>
        {#if haveChildFolders()}
            <Chevron
                type={state === NodeState.Collapsed
                    ? ChevronType.Right
                    : ChevronType.Down}
            />
        {/if}
    </div>
</div>

{#if haveChildFolders() && state === NodeState.Expanded}
    <div transition:slide={{ duration: 200, easing: quintOut }} class="ml-4">
        {#each folder.folders as childFolder}
            <svelte:self
                folder={childFolder}
                {selectedFolderId}
                on:selected={forwardSelected}
            />
        {/each}
    </div>
{/if}
