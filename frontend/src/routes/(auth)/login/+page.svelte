<script lang="ts">
  import LoginButton from './LoginButton.svelte'

  import { Login } from 'sveltegram'

  import tgIcon from '$lib/assets/Telegram_logo.png'
  import iuIcon from '$lib/assets/iu_icon.png'
</script>

<div class="mx-auto mt-20 p-5 flex flex-col gap-2">
  <h1 class="mb-4 mx-auto text-2xl font-semibold">Login</h1>
  <Login username="batts_tatar_bot" />
  <LoginButton>
    <img
      class="w-8 h-8"
      src={tgIcon}
      alt="Icon of an origami plane, telegram"
    />
    Telegram
  </LoginButton>
  <LoginButton>
    <img class="w-8 h-8 rounded-full" src={iuIcon} alt=""/>
    IU account
  </LoginButton>
</div>