<script lang="ts">
	import { goto } from "$app/navigation";
	import { page } from "$app/state";
  import { getIcon } from "$lib/icons"
	import { fade, slide } from "svelte/transition";

  let expanded = $state(false);

  const expand = () => { expanded = true};
  const compress = () => { expanded = false};

  const nav = [
    {
      title: "Play",
      icon: "gamepad",
      path: "/game"
    },
    {
      title: "Deck Builder",
      icon: "cards",
      path: "/deck-builder"
    },
    {
      title: "Settings",
      icon: "settings",
      path: "/settings"
    },
  ]
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_mouse_events_have_key_events -->
<div id="dock" class:expanded={expanded} onmouseover={expand} onmouseleave={compress}>
  {#each nav as item}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="item" onclick={() => goto(item.path)}>
    {#if page.url.pathname == item.path}
      <span class="icon selected">{@html getIcon(item.icon)}</span>
    {:else}
      <span class="icon">{@html getIcon(item.icon)}</span>
    {/if}
    {#if expanded}
      <span transition:slide={{ axis: "x", duration: 200 }} class="text">{item.title}</span>
    {/if}
  </div>
  {/each}
</div>
{#if expanded}
  <div id="blur" transition:fade={{ duration: 300 }}></div>
{/if}

<style lang="scss">
  #blur{
    z-index: 1;
    position: fixed;
    height: 100vh;
    width: 100vw;
    background-color: rgba($white, 0.45);
    backdrop-filter: blur(3px);
  }
  #dock{
    z-index: 2;
    transition: 500ms ease all;
    position: fixed;
    width: 50px;
    height: calc(100vh - 50px);
    margin-top: 50px;
    box-shadow: inset 0 10px 10px -10px rgba(0, 0, 0, 0.25);
    border-right: 1px solid $grey-mid;
    display: flex;
    gap: 25px;
    background-color: $white;
    width: 50px;
    overflow: hidden;
    .item{
      position: absolute;
      display: flex;
      justify-content: center;
      align-items: center;
      gap: 10px;
      left: 12px;
      overflow: hidden;

      &:hover{
        color: $primary;
        cursor: pointer;
      }
      
      @for $i from 1 through 8 {
        &:nth-child(#{$i}) {
          margin-top: calc(45px * $i - 15px);
        }
      }

      .text{
        display: flex;
        justify-content: center;
        align-items: center;
        white-space: nowrap;
      }

      .icon.selected{
        color: $primary;
      }

      &:last-child{
        position: absolute;
        bottom: 25px;
      }
    }
  }
  .expanded{
    width: 150px !important;
  }
</style>
