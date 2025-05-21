import Area from "@/models/area/area";
import { atom } from "jotai";

export const areaAtom = atom<Area>(Area.Tokyo);