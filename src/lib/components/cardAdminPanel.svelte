<script lang="ts">
  import { getIcon } from '$lib/icons.js';
  import { downloadDivAsPNG } from '$lib/util';
  let { card } = $props()
</script>

<div id="panel">
  <button class="btn download" onclick={()=>downloadDivAsPNG(card.name, card.name)}>
    <span class="icon">{@html getIcon("png")}</span>
  </button>
  <button class="btn new" onclick={()=>downloadDivAsPNG(card.name, card.name)}>
    <span class="icon">{@html getIcon("new")}</span>
  </button>
  <button class="btn edit" onclick={()=>downloadDivAsPNG(card.name, card.name)}>
    <span class="icon">{@html getIcon("creator")}</span>
  </button>
  <button class="btn delete" onclick={()=>downloadDivAsPNG(card.name, card.name)}>
    <span class="icon">{@html getIcon("delete")}</span>
  </button>
</div>


<style lang="scss">
  #panel{
    display: flex;
    flex-direction: row;
    justify-content: space-evenly;
    overflow: hidden;
    background-color: $grey-ultralight;
    border-radius: 5px;
    border: 1px solid $grey-dark;
    color: $black;
    min-width: 100%; 
    z-index: 2;
  }
  .icon{
    scale: 0.8;
  }
  .btn{
    width: 100%;
    border: none;
    background-color: inherit;
    align-self: center;
    height: 30px;

    &:hover{
      cursor: pointer;
      background-color: red;
    }
  }
  .download:hover { background-color: rgb(109, 145, 185); }
  .new:hover { background-color: rgb(87, 198, 116); }
  .delete:hover { background-color: rgb(207, 71, 71); }
  .edit:hover { background-color: rgb(234, 200, 80); }
</style>
