---
layout: home
title: "Conftier: Multi-level Configuration Framework for Python"
description: "A powerful configuration management system for Python applications with layered user and project settings, supporting Pydantic models and dataclasses."
head:

- - meta
  - property: og:title
    content: "Conftier: Multi-level Configuration Framework for Python"
- - meta
  - property: og:description
    content: "A powerful configuration management system for Python applications with layered user and project settings"
- - meta
  - name: keywords
    content: "conftier, python configuration, multi-level config, pydantic configuration, configuration framework, python library"

hero:
  name: "conftier"
  text: "Multi-level configuration framework"
  tagline: A powerful configuration management system for Python applications with layered user and project settings
  image:
    src: /logo.png
    alt: conftier
  actions:
    - theme: brand
      text: Get Started
      link: /guide/introduction
    - theme: alt
      text: View on GitHub
      link: <https://github.com/Undertone0809/conftier>

features:

- title: 🔄 Multi-level Configuration
  details: Intelligently merges configurations from default, user-level, and project-level sources with clear priority rules.
- title: 🧰 Framework-agnostic
  details: Compatible with multiple schema definition approaches including dataclasses, Pydantic models, and plain dictionaries.
- title: 🔐 Secure Configuration
  details: Safely store sensitive information like API keys at the user level, separate from version-controlled project files.
- title: 🔍 Type Safety
  details: Full type hints and validation ensure configuration correctness at runtime.
- title: 🚀 Zero Configuration
  details: Automatic creation of configuration files and directory structures - no complex setup needed.
- title: 🧩 Partial Updates
  details: Update specific parts of your configuration without affecting other settings.
