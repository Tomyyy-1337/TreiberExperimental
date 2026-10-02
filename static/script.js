// Load the theme from cookie on page load
// Prevents flickering when going back after changing the theme
// Theme is also applied immediately on page load
window.addEventListener('pageshow', function () {
    const theme = document.cookie
        .split('; ')
        .find((cookie) => cookie.startsWith('theme='))
        ?.split('=')[1];
    document.documentElement.setAttribute(
        'data-theme',
        theme === 'dark' ? 'dark' : 'light'
    );
});