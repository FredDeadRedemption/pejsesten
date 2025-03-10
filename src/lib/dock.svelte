<script lang="ts">
	import { goto } from "$app/navigation";
  import {getIcon} from "$lib/icons"
	import { fade, slide } from "svelte/transition";

  let expanded = $state(false);

  const expand = () => { expanded = true; console.log("mouse enter")};
  const compress = () => { expanded = false};
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_mouse_events_have_key_events -->
<div id="dock" class:expanded={expanded} onmouseover={expand} onmouseleave={compress}>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="item" onclick={() => goto("/game")}><span class="icon">{@html getIcon("gamepad")}</span>
    {#if expanded}
      <span transition:slide={{ axis: "x", duration: 300 }} class="text">Play</span>
    {/if}
  </div>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div onclick={() => goto("/deck-builder")} class="item"><span class="icon">{@html getIcon("cards")}</span>
    {#if expanded}
      <span transition:slide={{ axis: "x", duration: 300 }} class="text">Deck Builder</span>
    {/if}
  </div>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="item" onclick={() => goto("/settings")}><span class="icon">{@html getIcon("settings")}</span>
    {#if expanded}
      <span transition:slide={{ axis: "x", duration: 300 }} class="text">Settings</span>
    {/if}
  </div>
  </div>

<style lang="scss">
  .expanded{
    width: 150px !important;
  }
  #dock{
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
      
      @for $i from 1 through 5 {  // Adjust range as needed
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

      &:last-child{
        position: absolute;
        bottom: 25px;
      }
    }
  }
</style>
