import { faker } from '@faker-js/faker';
import fs from 'fs';
import { Database as mockConfig } from '@/types/supabase.types';

type GeneratorType = 'string' | 'date' | 'uuid' | 'boolean' | 'json';

const generators: Record<GeneratorType, () => any> = {
    string: () => faker.random.word(),
    date: () => faker.date.recent().toISOString(),
    uuid: () => faker.datatype.uuid(),
    boolean: () => faker.datatype.boolean(),
    json: () => ({ key: faker.random.word() }),
    // ... add more as needed
};

function generateMockData(config: Record<string, Record<string, Record<string, GeneratorType>>>) {
    const mockData: any = {};

    for (const tableName in config) {
        mockData[tableName] = {};
        for (const rowType in config[tableName]) {
            mockData[tableName][rowType] = {};
            const fields = config[tableName][rowType];
            for (const field in fields) {
                const generatorType = fields[field];
                const generator = generators[generatorType];
                if (generator) {
                    mockData[tableName][rowType][field] = generator();
                }
            }
        }
    }

    return mockData;
}

// const mockDatabaseData = generateMockData(mockConfig);

// Write the mock data to a JSON file
// fs.writeFileSync('mockData.json', JSON.stringify(mockDatabaseData, null, 2));

// console.log('Mock data generated and saved to mockData.json');


// console.log(mockDatabaseData);
