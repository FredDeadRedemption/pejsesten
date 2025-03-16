<script lang="ts">
  let { data } = $props()
  let { profile } = $derived(data)

</script>

<h1>dashboard</h1>
<p>Greetings, <strong>{profile?.username}!</strong> 
  Take a look at the <a href="/catalog">Catalog</a> 
or browse the <a href="/deck-builder">Deck Builder</a>,
you have <span class="wins">0</span> wins!&nbsp;(noob)</p>

<style lang="scss">
  p{ 
    color: $black;
  }
  .wins{
    color: $primary;
  }
  a{
    color: $primary;
  }
</style>