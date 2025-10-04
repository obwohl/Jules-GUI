from setuptools import setup, find_packages

setup(
    name='jcat',
    version='0.1.0',
    py_modules=['jcat'],
    include_package_data=True,
    install_requires=[
        'requests',
        'questionary',
    ],
    entry_points={
        'console_scripts': [
            'jcat = jcat:main',
        ],
    },
    author='Jules',
    author_email='jules@example.com',
    description='A fast and lean CLI for interacting with the Jules API.',
    long_description=open('README.md').read(),
    long_description_content_type='text/markdown',
    url='https://github.com/example/jcat',
    classifiers=[
        'Programming Language :: Python :: 3',
        'License :: OSI Approved :: MIT License',
        'Operating System :: OS Independent',
    ],
    python_requires='>=3.6',
)