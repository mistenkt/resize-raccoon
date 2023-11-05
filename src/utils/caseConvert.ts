function toSnakeCase(str: string): string {
    return str.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`);
}

function toCamelCase(str: string): string {
    return str.replace(/(_\w)/g, k => k[1].toUpperCase());
}

function convertKeysToSnakeCase(obj: any): any {
    if (typeof obj !== 'object' || obj === null) {
        return obj;
    }

    if (Array.isArray(obj)) {
        return obj.map(convertKeysToSnakeCase);
    }

    return Object.keys(obj).reduce((acc, key) => {
        const newKey = toSnakeCase(key);
        acc[newKey] = convertKeysToSnakeCase(obj[key]);
        return acc;
    }, {} as any);
}

function convertKeysToCamelCase(obj: any): any {
    if (typeof obj !== 'object' || obj === null) {
        return obj;
    }

    if (Array.isArray(obj)) {
        return obj.map(convertKeysToCamelCase);
    }

    return Object.keys(obj).reduce((acc, key) => {
        const newKey = toCamelCase(key);
        acc[newKey] = convertKeysToCamelCase(obj[key]);
        return acc;
    }, {} as any);
}

export default {
    toCamel: convertKeysToCamelCase,
    toSnake: convertKeysToSnakeCase
}