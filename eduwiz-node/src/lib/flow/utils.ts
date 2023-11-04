import { authenticate, unauthenticate, currentUser } from '@onflow/fcl'
import './config'
import { writable } from 'svelte/store'
import { browser } from '$app/environment'
import { goto } from '$app/navigation'

export async function logIn() {
  const user = await authenticate()
  if (user.loggedIn) {
    goto('/home')
  }
}

export async function logOut() {
  await unauthenticate()
  goto('/')
}

export const user = writable({
  loggedIn: false,
  addr: null,
  username: null,
})

if (browser) {
  currentUser.subscribe(user.set)
}
