import fsPkg from 'fs-extra';
const { rmdir } = fsPkg;

(async () => {
    await rmdir('dist', { recursive: true });
})();


