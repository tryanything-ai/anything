import { faker } from '@faker-js/faker';
import fs from 'fs';
import { Database, Database as mockConfig } from '@/types/supabase.types';

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

type Profile = Database['public']['Tables']['profiles']['Row'];

export const FakeProfiles: Profile[] = [
    {
        avatar_url: "https://fokcbrnvhnwnwwpiqkdc.supabase.co/storage/v1/object/public/mocks/dumbledore.jpeg",
        full_name: "Albus Percival Wulfric Brian Dumbledore",
    bio: "Headmaster of Hogwarts School of Witchcraft and Wizardry",    
      public: true,
    github: null,
        id: "1",
        instagram: "albus_insta_magic",
        linkedin: null,
        tiktok: "albus_tiktok_spells",
        twitter: "headmasterAlbus",
        updated_at: "2023-10-09",
        username: "dumbledore",
        website: "https://hogwarts.edu/faculty/dumbledore",
        youtube: "DumbledoreMagicChannel"
      },
      {
        avatar_url: "https://fokcbrnvhnwnwwpiqkdc.supabase.co/storage/v1/object/public/mocks/harry.webp",
        full_name: "Harry James Potter",
        bio: "Student at Hogwarts School of Witchcraft and Wizardry",
        public: true,
        github: null,
        id: "2",
        instagram: "theboywholived_official",
        linkedin: null,
        tiktok: "lightning_scar_tiktok",
        twitter: "real_harrypotter",
        updated_at: "2023-10-09",
        username: "harry",
        website: "https://hogwarts.edu/students/harrypotter",
        youtube: "PotterQuidditchPlays"
      },
      // {
      //   avatar_url: "https://fokcbrnvhnwnwwpiqkdc.supabase.co/storage/v1/object/public/mocks/harry.webp",
      //   full_name: "Hermione Jean Granger",
      //   github: null,
      //   bio: "Student at Hogwarts School of Witchcraft and Wizardry",
      //   public: true, 
      //   id: "3",
      //   instagram: "smartwitch_hermione",
      //   linkedin: "hermione_professional",
      //   tiktok: null,
      //   twitter: "bookworm_hermione",
      //   updated_at: "2023-10-09",
      //   username: "hermione_g",
      //   website: "https://hogwarts.edu/students/hermionegranger",
      //   youtube: "HermioneStudyGuides"
      // },
      // {
      //   avatar_url: "https://hogwarts.edu/avatars/ron_weasley.jpg",
      //   full_name: "Ronald Bilius Weasley",
      //   bio: "Student at Hogwarts School of Witchcraft and Wizardry",
      //   github: null,
      //   id: "4",
      //   instagram: "weasleyisourking_ron",
      //   linkedin: null,
      //   tiktok: "ron_tiktok_chess",
      //   twitter: "ron_the_king",
      //   updated_at: "2023-10-09",
      //   username: "ron_weasley",
      //   website: "https://hogwarts.edu/students/ronweasley",
      //   youtube: "WeasleyJokes"
      // },
      // {
      //   avatar_url: "https://hogwarts.edu/avatars/draco_malfoy.jpg",
      //   full_name: "Draco Lucius Malfoy",
      //   github: null,
      //   id: "5",
      //   instagram: "pureblood_draco",
      //   linkedin: "draco_malfoy_enterprises",
      //   tiktok: null,
      //   twitter: "slytherin_prince",
      //   updated_at: "2023-10-09",
      //   username: "draco_m",
      //   website: "https://hogwarts.edu/students/dracomalfoy",
      //   youtube: "MalfoyManorTours"
      // }
      
]