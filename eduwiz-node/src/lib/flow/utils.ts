import * as fcl from '@onflow/fcl'
import './config'
import { writable } from 'svelte/store'
import { browser } from '$app/environment'
import { goto } from '$app/navigation'

export async function logIn() {
  const u = await fcl.authenticate()
  if (u.loggedIn) {
    goto('/home')
  }
}

export async function logOut() {
  await fcl.unauthenticate()
  goto('/')
}

export async function getAccount() {
  const address = (await fcl.currentUser.snapshot()).addr;
  return (await fcl.send([await fcl.getAccount(address)])).account;
}

export const user = writable({
  loggedIn: false,
  addr: null,
  username: null,
})

if (browser) {
  fcl.currentUser.subscribe(user.set)
}
