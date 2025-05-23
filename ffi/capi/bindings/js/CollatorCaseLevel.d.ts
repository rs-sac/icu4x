// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/**
 * See the [Rust documentation for `CaseLevel`](https://docs.rs/icu/latest/icu/collator/options/enum.CaseLevel.html) for more information.
 */


export class CollatorCaseLevel {

    static fromValue(value: CollatorCaseLevel | string): CollatorCaseLevel;

    get value(): string;

    get ffiValue(): number;

    static Off : CollatorCaseLevel;
    static On : CollatorCaseLevel;


    constructor(value: CollatorCaseLevel | string );
}