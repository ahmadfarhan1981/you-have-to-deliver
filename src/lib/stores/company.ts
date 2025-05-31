import {writable} from "svelte/store";

export type CompanySnapshot = {
    name:string,
    slogan:string,
}


export let company = writable<CompanySnapshot>({name: "", slogan: ""});
export const companySnapshotEventName ="company_snapshot";