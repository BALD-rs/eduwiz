import { authenticate, unauthenticate, currentUser } from '@onflow/fcl'
import './config'
import { writable } from 'svelte/store'
import { browser } from '$app/environment'

export async function logIn() {
  authenticate()
}

export async function logOut() {
  unauthenticate()
}

export const user = writable({
  loggedIn: false,
  addr: null,
  username: null,
})

if (browser) {
  currentUser.subscribe(user.set)
}
