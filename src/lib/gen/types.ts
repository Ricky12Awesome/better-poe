/*
 Generated by typeshare 1.7.0
*/

export interface Token {
	access_token: string;
	expires_in: number;
}

export interface State {
	last_page: number;
}

export interface Settings {
	log_file?: string;
	theme?: string;
	token?: Token;
}

