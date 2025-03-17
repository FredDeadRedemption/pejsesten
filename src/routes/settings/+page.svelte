<script>
</script>

<main>
  <h1>Settings</h1>
  <div class="wrap">
    <p>Session server runs on <strong>(This is broken)</strong></p> 
    <div class="toggle-switch">
      <label class="switch">
        <input type="checkbox" checked />
        <span class="slider"></span>
      </label>
    </div>
  </div>
  <div class="sep"></div>
  <div class="wrap">
    <p>Activate the <strong>'Unga Bunga'</strong> protocol</p> 
    <div class="toggle-switch">
      <label class="switch">
        <input type="checkbox" checked/>
        <span class="slider"></span>
      </label>
    </div>
  </div>
  <div class="sep"></div>
  <div class="wrap">
    <p>Enable <strong>Schrödinger</strong> mode</p> 
    <div class="toggle-switch">
      <label class="switch">
        <input class="schrodinger-input" type="checkbox" />
        <span class="slider schrodinger"></span>
      </label>
    </div>
  </div>
</main>

<style lang="scss">
  main{
    margin-top: 2rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
    h1{
      margin-bottom: 15px;
    }
  }
  .wrap{
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 400px;
  }
  .sep{
    background-color: $grey-mid;
    height: 2px;
    width: 400px;
  }
  .toggle-switch {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .switch {
    position: relative;
    display: inline-block;
    width: 47px;
    height: 25px;
  } 

  .switch input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: #ccc;
    transition: 0.4s;
    border-radius: 2px;
  }

  .slider:before {
    position: absolute;
    content: '';
    height: 19px;
    width: 19px;
    left: 3px;
    bottom: 3px;
    background-color: white;
    transition: 0.4s;
    border-radius: 2px;
  }

  input:checked + .slider {
    background-color: $primary
  }

  input:checked + .slider.schrodinger {
    background-color: rgba($color: $primary, $alpha: 0.5);
  }

  input:checked + .slider:before {
    transform: translateX(21px);
  }

  input:checked + .slider.schrodinger:before {
    transform: translateX(10.5px); /* Move only 10px to the right */
  }
</style>