import { redirect } from '@sveltejs/kit';
import type { Actions } from './$types';

export const actions = {
	default: async (event) => {
		//TODO: UPDATE THIS TO GET PUT IN TO SET USER FUNCTION CURRENTLY LOGS TO CONSOLE
		console.log((await event.request.formData()).get('username'));
		throw redirect(303, '/home');
	}
} satisfies Actions;
