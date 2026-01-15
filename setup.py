#!/usr/bin/env python3
"""Setup script for Smile emoticon picker"""

from setuptools import setup

setup(
    name='smile',
    version='1.0.0',
    description='A simple emoticon picker for Gnome/Linux',
    author='Smile Contributors',
    py_modules=['smile'],
    install_requires=[
        'PyGObject>=3.42.0',
    ],
    entry_points={
        'console_scripts': [
            'smile=smile:main',
        ],
    },
    python_requires='>=3.6',
    classifiers=[
        'Development Status :: 4 - Beta',
        'Environment :: X11 Applications :: GTK',
        'Intended Audience :: End Users/Desktop',
        'License :: OSI Approved :: MIT License',
        'Operating System :: POSIX :: Linux',
        'Programming Language :: Python :: 3',
        'Topic :: Desktop Environment :: Gnome',
    ],
)
