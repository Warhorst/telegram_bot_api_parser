#telegram_bot_api_parser
Important: I currently paused the development of this software. 
It was primary a project to learn something new. 
I may continue it in the future. What is currently possible and what is not is listed below. 


##Summary
The goal of this tool is to generate code or api specification files (like swagger) from the Telegram-Bot-API-documentation (https://core.telegram.org/bots/api).
To achieve this, the api-HTML is processed and its content is parsed into a generalized format. 
The resulting structs can than be used in handlebars-templates (https://handlebarsjs.com/) to create the desired output.

##Current Features
Reading the data transfer objects (DTOs) from the api doc and using them in templates. An exception is listed in the missing features section.

Reading the methods from the api doc and using them in templates. Currently only the method name and return type is available. More in the missing features section.

Setting the desired values for the types used by the bot api. For example Boolean can be set to bool in the resulting code. The values for arrays and optionals can also contain templates.

Renaming a dto or method name with templates. Useful for rust for example, because the name "type" is widely used by dtos but is a reserved word.

Resolving templates for each dto/method or for the whole list.

##Missing Features
The main problem are methods. The return type is only written in the description, so a proper parsing strategy is currently not available.
There is also no strategy for resolving multiple possible parameter types, like "String or InputFile". The only exception is "String or Integer", where String is prefered.

The api-HTML must be downloaded and provided as a file.

DTOs and methods with no fields or parameters are currently ignored. h4 headers with no following table are currently expected to be "just headers".

More configuration options and more control via the cli.

A proper usage documentation. I may add somthing like this is this software will be fully functional. 