<script>
</script>

<form method="POST" action="?/login">
  <input name="email" type="email" placeholder="EMAIL" />
  <input name="password" type="password" placeholder="PASSWORD" />
  <button class="login">LOGIN</button>
  <button class="create" formaction="?/signup">SIGN UP</button>
</form>

<style lang="scss">

  form{
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    display: flex;
    flex-direction: column;
    border-radius: 4px;
    padding: 25px;
    gap: 25px;
    background-color: rgba($color: $white, $alpha: 0.2);
    backdrop-filter: blur(5px);
    border: 1px solid $white;
    input, button{
      width: 250px;
      padding: 10px;
      border-radius: 4px;
    }
    input{  
      border: 1px solid $white;
      background-color: transparent;  
      outline: none;
      color: $white;
    }
    button{
      border: none;
      color: $white;
      &:hover{
        cursor: pointer;
      }
    }
  }
  .login{
    background-color: $primary;
  }
  .create{
    background-color: $secondary;
  }
</style>