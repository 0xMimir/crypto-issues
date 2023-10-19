import axios from "axios";

export const client = axios.create({
	baseURL: import.meta.env.API_URL ? import.meta.env.API_URL : "http://localhost:1111"
});