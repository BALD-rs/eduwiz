import { redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { setUsername } from '$lib/flow/utils';

export const actions = {
	default: async (event) => {
		const formData = await event.request.formData();
		const username = formData.get('username');
		if (typeof username === 'string') {
			await setUsername(username);
		} else {
			console.log('Username is not provided or not a string');
		}
		throw redirect(303, '/home');
	}
} satisfies Actions;
