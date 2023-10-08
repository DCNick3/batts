import * as ts from 'typescript';
import * as path from 'path';

const indexJs = require.resolve('ts-transformer-dates');
const indexTs = path.join(path.dirname(indexJs), 'index.d.ts');

const factory = ts.factory;

export default function transformer(program: ts.Program): ts.TransformerFactory<ts.SourceFile> {
    return (context: ts.TransformationContext) => (file: ts.SourceFile) => {
        const toDatesByArray = factory.createUniqueName('toDatesByArray');

        const newFile = visitNodeAndChildren(file, program, context, toDatesByArray)


        return factory.updateSourceFile(newFile, [
            factory.createImportDeclaration(
                /* modifiers */ undefined,
                factory.createImportClause(
                    false,
                    undefined,
                    factory.createNamedImports([
                        factory.createImportSpecifier(false, undefined, toDatesByArray),
                    ]),
                ),
                factory.createStringLiteral('ts-transformer-dates'),
                null
            ),
            // Ensures the rest of the source files statements are still defined.
            ...newFile.statements,
        ])
    };
}

function visitNodeAndChildren(node: ts.SourceFile, program: ts.Program, context: ts.TransformationContext, toDatesByArray: ts.Identifier): ts.SourceFile;
function visitNodeAndChildren(node: ts.Node, program: ts.Program, context: ts.TransformationContext, toDatesByArray: ts.Identifier): ts.Node | undefined;
function visitNodeAndChildren(node: ts.Node, program: ts.Program, context: ts.TransformationContext, toDatesByArray: ts.Identifier): ts.Node | undefined {
    return ts.visitEachChild(visitNode(node, program, toDatesByArray), childNode => visitNodeAndChildren(childNode, program, context, toDatesByArray), context);
}

function visitNode(node: ts.SourceFile, program: ts.Program, toDatesByArray: ts.Identifier): ts.SourceFile;
function visitNode(node: ts.Node, program: ts.Program, toDatesByArray: ts.Identifier): ts.Node | undefined;
function visitNode(node: ts.Node, program: ts.Program, toDatesByArray: ts.Identifier): ts.Node | undefined {
    const typeChecker = program.getTypeChecker();

    if (isToDatesExpression(node, typeChecker)) {
        console.log("GOTCHA!!!!")

        if (!node.typeArguments) {
            console.warn('toDates call expression without type arguments', node);
            return node;
        }

        const type = typeChecker.getTypeFromTypeNode(unbox(node.typeArguments[0]));
        const toDatesByArrayArgs = [
            node.arguments[0],
            factory.createArrayLiteralExpression(convertDates(type, typeChecker, [], node))
        ];
        if (node.arguments.length > 1) toDatesByArrayArgs.push(node.arguments[1]);
        return factory.createCallExpression(
            toDatesByArray,
            undefined,
            toDatesByArrayArgs
        );
    } else {
        return node;
    }
}

const isToDatesExpression = (node: ts.Node, typeChecker: ts.TypeChecker): node is ts.CallExpression => {
    if (!ts.isCallExpression(node)) return false;

    const signature = typeChecker.getResolvedSignature(node);
    if (typeof signature === 'undefined') {
        return false;
    }

    const { declaration } = signature;

    return (
        !!declaration &&
        !ts.isJSDocSignature(declaration) &&
        require.resolve(declaration.getSourceFile().fileName) === indexTs &&
        !!declaration.name &&
        declaration.name.getText() === 'toDates'
    );
};

function unbox(typeNode: ts.TypeNode) {
    while (ts.isArrayTypeNode(typeNode)) {
        typeNode = (typeNode as ts.ArrayTypeNode).elementType;
    }
    return typeNode;
}

function convertDates(
    type: ts.Type,
    typeChecker: ts.TypeChecker,
    prefix: ts.StringLiteral[],
    node: ts.Node
): ts.ArrayLiteralExpression[] {
    const properties = typeChecker.getPropertiesOfType(type);
    const getTypeOfProperty = (property: ts.Symbol) => {
        const propertyType = unbox((property.valueDeclaration as ts.PropertyDeclaration)?.type as ts.TypeNode);
        return typeChecker.getTypeFromTypeNode(propertyType).getNonNullableType();
    };
    return properties
        .reduce((props, property) => {
            const propertyType = getTypeOfProperty(property);
            console.log(typeChecker.typeToString(propertyType));
            if (typeChecker.typeToString(propertyType) === 'Date') {
                return props.concat(factory.createArrayLiteralExpression(prefix.concat([factory.createStringLiteral(property.getName())])));
            }
            if (propertyType.isClassOrInterface()) {
                return props.concat(
                    convertDates(
                        propertyType,
                        typeChecker,
                        prefix.concat(factory.createStringLiteral(property.getName())),
                        node
                    )
                );
            }
            return props;
        }, [] as ts.ArrayLiteralExpression[]);
}