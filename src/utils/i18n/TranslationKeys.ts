import en from "../../locales/en";

type NestedKeyOf<ObjectType extends object> = {
    [Key in keyof ObjectType &
        (string | number)]: ObjectType[Key] extends object
        ? `${Key}.${NestedKeyOf<ObjectType[Key]>}`
        : Key;
}[keyof ObjectType & (string | number)];

type TranslationKeys = NestedKeyOf<typeof en>;

export default TranslationKeys;