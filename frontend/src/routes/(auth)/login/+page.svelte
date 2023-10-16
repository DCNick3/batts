<script lang="ts">
  import LoginButton from './LoginButton.svelte'
  import { Login } from 'sveltegram'
  import { goto, invalidateAll } from '$app/navigation'
  import { Api } from 'backend'

  import iuIcon from '$lib/assets/iu_icon.png'

  let state: 'Ok' | 'Error' = 'Ok'
  let errorMessage = ''

  const api = new Api(fetch)
  const handleLogin = async (data: any) => {
    try {
      const result = await api.telegramLogin(data.detail)
      // const result = { status: 'Success', payload: null}
      if (result.status === 'Success') {
        state = 'Ok'
        await invalidateAll()
        await goto('/')
      } else {
        // TODO: handle error
        state = 'Error'
        errorMessage = result.payload.report
      }
    } catch (error) {
      // TODO: handle error
      state = 'Error'
      errorMessage = 'Failed to connect'
    }
  }
</script>

<div class="mx-auto mt-20 p-5 flex flex-col gap-2">
  <h1 class="mb-4 mx-auto text-2xl font-semibold">Login</h1>
  <Login username="batts_tatar_bot" on:auth={handleLogin} />
  <LoginButton>
    <img class="w-8 h-8 rounded-full" src={iuIcon} alt=""/>
    IU account
  </LoginButton>
</div>

{#if state === 'Error'}
  <h1 class="text-red-500">Login Failure: {errorMessage}</h1>
{/if}