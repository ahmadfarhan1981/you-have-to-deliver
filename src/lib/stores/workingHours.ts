import { writable } from "svelte/store";

export const workingHoursEventName = "monthly_availability_snapshot";

export type SimDay = {
  year: number;
  week: number;
  day: number;
};

export type WorkingDaySnapshot = {
  date: SimDay;
  slots: boolean[];
};

export type WorkingHoursSnapshot = {
  person_id: number;
  days: WorkingDaySnapshot[];
};

export const workingHours = writable<WorkingHoursSnapshot[]>([]);
