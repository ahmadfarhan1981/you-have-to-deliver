import type {AssignedSkillSnapshot} from "$lib/stores/persons";

export type ProfilePictureSnapshot = {
    gender: string;
    category: number; // i8 in Rust maps to number in TS
    batch: number;
    index: number;
};

export function getProfileImageData(pic: ProfilePictureSnapshot) {
    const gender = pic.gender.slice(0, 1) === "f" ? "f" : "m";
    const pad = (num: number) => String(Math.min(99, Math.max(0, Math.floor(num)))).padStart(2, "0");
    const category = pad(pic.category);
    const batch = pad(pic.batch);
    const index = Math.max(0, Math.min(8, pic.index));
    const fileName = `${gender}${category}${batch}.png`;
    const col = index % 3;
    const row = Math.floor(index / 3);
    const position = `${col * 50}% ${row * 50}%`;
    return {fileName, position};
}

export type SkillSetSnapshot = {
    assigned_skills: AssignedSkillSnapshot[];
};