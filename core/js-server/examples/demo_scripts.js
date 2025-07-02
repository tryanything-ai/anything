// Demo JavaScript Examples for the Rust+Deno Executor
// These examples show what users can write in your automation system

// Example 1: Simple data transformation
function simpleTransformation() {
    return {
        original: inputs.value,
        doubled: inputs.value * 2,
        squared: inputs.value * inputs.value,
        timestamp: new Date().toISOString()
    };
}

// Example 2: Array processing and filtering
function processUserData() {
    const users = inputs.users;
    
    const activeUsers = users.filter(user => user.active);
    const usersByAge = users.sort((a, b) => b.age - a.age);
    const averageAge = users.reduce((sum, user) => sum + user.age, 0) / users.length;
    
    return {
        total_users: users.length,
        active_users: activeUsers.length,
        oldest_user: usersByAge[0],
        youngest_user: usersByAge[usersByAge.length - 1],
        average_age: averageAge,
        active_user_names: activeUsers.map(user => user.name)
    };
}

// Example 3: Complex business logic
function calculateOrderSummary() {
    const orders = inputs.orders;
    
    const summary = orders.reduce((acc, order) => {
        const orderTotal = order.items.reduce((itemSum, item) => {
            return itemSum + (item.price * item.quantity);
        }, 0);
        
        const tax = orderTotal * 0.08; // 8% tax
        const finalTotal = orderTotal + tax;
        
        acc.total_orders++;
        acc.gross_revenue += orderTotal;
        acc.tax_collected += tax;
        acc.net_revenue += finalTotal;
        
        if (order.customer_type === 'premium') {
            acc.premium_orders++;
            acc.premium_revenue += finalTotal;
        }
        
        return acc;
    }, {
        total_orders: 0,
        gross_revenue: 0,
        tax_collected: 0,
        net_revenue: 0,
        premium_orders: 0,
        premium_revenue: 0
    });
    
    summary.average_order_value = summary.net_revenue / summary.total_orders;
    summary.premium_percentage = (summary.premium_orders / summary.total_orders) * 100;
    
    return summary;
}

// Example 4: Data validation and cleaning
function cleanAndValidateData() {
    const rawData = inputs.data;
    
    const cleaned = rawData
        .filter(item => item !== null && item !== undefined)
        .map(item => {
            // Clean and normalize the data
            const cleaned = {
                id: item.id,
                name: typeof item.name === 'string' ? item.name.trim() : '',
                email: typeof item.email === 'string' ? item.email.toLowerCase().trim() : '',
                age: typeof item.age === 'number' ? Math.max(0, Math.min(150, item.age)) : null,
                created_at: item.created_at ? new Date(item.created_at).toISOString() : new Date().toISOString()
            };
            
            // Validate email format
            const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
            cleaned.email_valid = emailRegex.test(cleaned.email);
            
            // Validate required fields
            cleaned.is_valid = cleaned.name.length > 0 && cleaned.email_valid && cleaned.age !== null;
            
            return cleaned;
        });
    
    const valid = cleaned.filter(item => item.is_valid);
    const invalid = cleaned.filter(item => !item.is_valid);
    
    return {
        total_processed: rawData.length,
        valid_records: valid.length,
        invalid_records: invalid.length,
        valid_data: valid,
        invalid_data: invalid,
        validation_rate: (valid.length / rawData.length) * 100
    };
}

// Example 5: Time-based data analysis
function analyzeTimeSeriesData() {
    const data = inputs.timeseries;
    
    // Group data by day
    const dailyData = data.reduce((acc, point) => {
        const date = new Date(point.timestamp).toISOString().split('T')[0];
        if (!acc[date]) {
            acc[date] = [];
        }
        acc[date].push(point.value);
        return acc;
    }, {});
    
    // Calculate daily statistics
    const dailyStats = Object.entries(dailyData).map(([date, values]) => {
        const sum = values.reduce((a, b) => a + b, 0);
        const avg = sum / values.length;
        const min = Math.min(...values);
        const max = Math.max(...values);
        const variance = values.reduce((acc, val) => acc + Math.pow(val - avg, 2), 0) / values.length;
        const stdDev = Math.sqrt(variance);
        
        return {
            date,
            count: values.length,
            sum,
            average: avg,
            min,
            max,
            variance,
            standard_deviation: stdDev
        };
    });
    
    // Overall statistics
    const allValues = data.map(point => point.value);
    const overallAvg = allValues.reduce((a, b) => a + b, 0) / allValues.length;
    
    return {
        total_points: data.length,
        date_range: {
            start: dailyStats[0]?.date,
            end: dailyStats[dailyStats.length - 1]?.date
        },
        overall_average: overallAvg,
        daily_statistics: dailyStats,
        trend: dailyStats.length > 1 ? 
            (dailyStats[dailyStats.length - 1].average > dailyStats[0].average ? 'increasing' : 'decreasing') : 
            'insufficient_data'
    };
}

// Example 6: String processing and text analysis
function analyzeText() {
    const text = inputs.text;
    
    // Basic text statistics
    const words = text.toLowerCase().match(/\b\w+\b/g) || [];
    const sentences = text.split(/[.!?]+/).filter(s => s.trim().length > 0);
    const paragraphs = text.split(/\n\s*\n/).filter(p => p.trim().length > 0);
    
    // Word frequency
    const wordFreq = words.reduce((acc, word) => {
        acc[word] = (acc[word] || 0) + 1;
        return acc;
    }, {});
    
    // Most common words
    const commonWords = Object.entries(wordFreq)
        .sort(([,a], [,b]) => b - a)
        .slice(0, 10)
        .map(([word, count]) => ({ word, count }));
    
    // Reading time estimation (average 200 words per minute)
    const readingTimeMinutes = Math.ceil(words.length / 200);
    
    return {
        character_count: text.length,
        word_count: words.length,
        sentence_count: sentences.length,
        paragraph_count: paragraphs.length,
        average_words_per_sentence: words.length / sentences.length,
        unique_words: Object.keys(wordFreq).length,
        most_common_words: commonWords,
        estimated_reading_time_minutes: readingTimeMinutes,
        text_complexity: words.length / sentences.length > 20 ? 'complex' : 'simple'
    };
}

// Example 7: Mathematical calculations
function performCalculations() {
    const numbers = inputs.numbers;
    
    // Basic statistics
    const sum = numbers.reduce((a, b) => a + b, 0);
    const mean = sum / numbers.length;
    const median = [...numbers].sort((a, b) => a - b)[Math.floor(numbers.length / 2)];
    const mode = numbers.reduce((acc, num) => {
        acc[num] = (acc[num] || 0) + 1;
        return acc;
    }, {});
    
    const mostFrequent = Object.entries(mode).reduce((a, b) => mode[a[0]] > mode[b[0]] ? a : b);
    
    // Advanced calculations
    const variance = numbers.reduce((acc, num) => acc + Math.pow(num - mean, 2), 0) / numbers.length;
    const standardDeviation = Math.sqrt(variance);
    
    // Quartiles
    const sorted = [...numbers].sort((a, b) => a - b);
    const q1 = sorted[Math.floor(sorted.length * 0.25)];
    const q3 = sorted[Math.floor(sorted.length * 0.75)];
    const iqr = q3 - q1;
    
    return {
        count: numbers.length,
        sum,
        mean,
        median,
        mode: mostFrequent[0],
        range: Math.max(...numbers) - Math.min(...numbers),
        variance,
        standard_deviation: standardDeviation,
        quartiles: { q1, q3, iqr },
        outliers: numbers.filter(n => n < q1 - 1.5 * iqr || n > q3 + 1.5 * iqr)
    };
}

// Export examples for testing
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        simpleTransformation,
        processUserData,
        calculateOrderSummary,
        cleanAndValidateData,
        analyzeTimeSeriesData,
        analyzeText,
        performCalculations
    };
} 