<script lang="ts">
  let { graveyard = $bindable() } = $props();

</script>

<div id="wrapper">
  <div id="graveyard">
    {graveyard.length}
  </div>
</div>

<style lang="scss">
  #wrapper{ 
    height: 100%;
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    #graveyard{
      height: 147px;
      border: 3px solid grey;
      width: 100px;
      &:hover{
        cursor: pointer;
      }
    }
  }
</style>