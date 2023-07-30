function calculateDaysBetweenDates(begin, end) {
    const beginDate = new Date(begin);
    const endDate = new Date(end);
    const millisecondsPerDay = 24 * 60 * 60 * 1000;
    return Math.round((endDate - beginDate) / millisecondsPerDay);
}

